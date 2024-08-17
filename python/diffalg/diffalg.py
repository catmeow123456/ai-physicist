from diffalg.expr_manipulate import sympify
from diffalg.mapleIO import mapleIO
import sympy as sp
from typing import List, Dict, Union, Tuple, Literal


class DifferentialRing:
    # derivations define the arguments of the functions
    derivations: List[sp.Symbol]
    # blocks define the block order of the variables
    blocks: List[Tuple[str, List[Union[sp.Symbol, sp.Function]]]]
    # some dictionaries to convert the names of the variables
    _name_dict: Dict[str, str]  # {'pos' : 'pos(t)', 'posr' : 'posr(t)', 'mass' : 'mass'}
    _name_dict_2: Dict[str, str]  # {'pos(t)' : 'pos', 'posr(t)' : 'posr', 'mass' : 'mass()'}

    def __init__(self, blocks: List[Tuple[str, List[Union[sp.Symbol, sp.Function]]]]):
        self.blocks = blocks
        derivs = set()
        self._name_dict = {}
        self._name_dict_2 = {}
        for item in blocks:
            assert item[0] in ['grlexA', 'grlexB', 'degrevlexA', 'degrevlexB', 'lex']
            # assert len(item[1]) > 0
            for var in item[1]:
                if self._name_dict.__contains__(var.name):
                    raise ValueError(f'Error! Variable {var.name} appears multiple times!')
                temp = str(var).replace(' ', '')
                self._name_dict[var.name] = temp
                self._name_dict_2[var.name] = (temp + '()') if var.is_Symbol else temp
                if var.is_Function:
                    derivs |= set(var.args)
        self.derivations = list(derivs)
        for var in self.derivations:
            if self._name_dict.__contains__(var.name):
                raise ValueError(f'Error! Variable {self._name_dict[var.name]} in blocks contradicts with argument {var.name}')
            temp = var.name.replace(' ', '')
            self._name_dict[var.name] = temp
            self._name_dict_2[var.name] = temp

    @classmethod
    def default(cls, vars: List[Union[sp.Symbol, sp.Function]]):
        return cls([('lex', vars)])

    def ring_to_maple(self, trans_table: Literal['ver1', 'ver2'] = 'ver1') -> str:
        derivs_arg = '[' + ', '.join([deriv.name for deriv in self.derivations]) + ']'
        if (trans_table == 'ver1'):
            blocks_arg = '[' + ', '.join([block[0] + '[' +
                                          ','.join([var.name for var in block[1]]) +
                                          ']' for block in self.blocks]) + ']'
        else:
            blocks_arg = '[' + ', '.join(['[' +
                                          ','.join([self._name_dict_2[var.name] for var in block[1]]) +
                                          ']' for block in self.blocks]) + ']'
        return f'DifferentialRing(blocks = {blocks_arg}, derivations = {derivs_arg})'


class RegularDifferentialChain:
    ring : DifferentialRing
    gb: List[sp.Expr]
    def __init__(self, ring: DifferentialRing, gb: List[sp.Expr] = None):
        self.ring = ring
        self.gb = gb if gb is not None else []

    def __str__(self):
        return '[' + ', '.join([str(i) for i in self.gb]) + ']'

    def reduce(self, eq: sp.Expr) -> sp.Expr:
        if not self.gb:
            return eq
        solver = mapleIO()
        solver.import_lib('DifferentialAlgebra')
        solver.import_lib('Tools')
        solver.append_command(f'R := {self.ring.ring_to_maple(trans_table="ver2")}')
        eqs_arg = ', '.join([eq_to_maple(self.ring, i, trans_table="ver2") for i in self.gb])
        solver.append_command(f'eqs := [{eqs_arg}]')
        solver.append_command(f'ideal := PretendRegularDifferentialChain(eqs, R)')
        eq_arg = eq_to_maple(self.ring, eq=eq, trans_table="ver2")
        solver.append_command(f'eq := ReducedForm({eq_arg}, ideal)')
        solver.append_command(f'print(eq)')
        stdout = solver.exec_maple()
        return eq_from_maple(self.ring, stdout[-1])


class diffalg:
    ring: DifferentialRing
    gb: List[RegularDifferentialChain]
    eqs: List[sp.Expr]
    ineqs: List[sp.Expr]

    def __init__(self, ring: DifferentialRing, gb: List[RegularDifferentialChain] = None,
                 eqs: List[sp.Expr] = None, ineqs: List[sp.Expr] = None):
        self.ring = ring
        self.gb = [RegularDifferentialChain(ring)] if gb is None else gb
        self.eqs = [] if eqs is None else eqs
        self.ineqs = [] if ineqs is None else ineqs

    def __str__(self):
        return 'DifferentialAlgebra:\n' + '\n'.join([i.__str__() for i in self.gb])

    @classmethod
    def from_eqs(cls, ring: DifferentialRing, eqs: List[sp.Expr] = None, ineqs: List[sp.Expr] = None):
        if eqs is None or eqs == []:
            return cls(ring, gb=None, eqs=None, ineqs=ineqs)
        if ineqs is None:
            ineqs = []
        solver = mapleIO()
        solver.import_lib('DifferentialAlgebra')
        solver.append_command(f'R := {ring.ring_to_maple()}')
        eqs_args = [eq_to_maple(ring, i) for i in eqs] + [eq_to_maple(ring, i) + '<> 0' for i in ineqs]
        args = ', '.join(eqs_args)
        solver.append_command(f'eqs := [{args}]')
        solver.append_command(f'ideal := RosenfeldGroebner(eqs, R)')
        solver.append_command(f'print(Equations(ideal))')
        stdout = solver.exec_maple()
        return cls(ring, gb_from_maple(ring, stdout[-1]), eqs, ineqs)

    def insert_new_eq(self, eq: sp.Expr) -> 'diffalg':
        if self.belongs_to(eq):
            return self
        return diffalg.from_eqs(self.ring, self.eqs + [eq], self.ineqs)

    def insert_new_ineqs(self, eq: sp.Expr) -> 'diffalg':
        if eq in self.ineqs:
            return self
        if self.belongs_to(eq):
            return diffalg(self.ring, gb=None, eqs=self.eqs, ineqs=self.ineqs + [eq])
        return diffalg.from_eqs(self.ring, self.eqs, self.ineqs + [eq])

    def belongs_to(self, eq: sp.Expr) -> bool:
        symbols = set()
        for i in self.gb:
            for j in i.gb:
                symbols |= set(j.free_symbols)
        if not eq.free_symbols.issubset(symbols):
            return False
        solver = mapleIO()
        solver.import_lib('DifferentialAlgebra')
        solver.import_lib('Tools')
        solver.append_command(f'R := {self.ring.ring_to_maple(trans_table="ver2")}')
        arg_lst = []
        for ideal in self.gb:
            eqs_arg = ', '.join([eq_to_maple(self.ring, i, trans_table="ver2") for i in ideal.gb])
            arg_lst.append(f'PretendRegularDifferentialChain([{eqs_arg}], R)')
        arg = ', '.join(arg_lst)
        solver.append_command(f'ideal := [{arg}]')
        eq_arg = eq_to_maple(self.ring, eq=eq, trans_table="ver2")
        solver.append_command(f'print(BelongsTo([{eq_arg}], ideal))')
        stdout = solver.exec_maple()
        result = stdout[-1].strip('[] ')
        assert result in ['true', 'false']
        return result == 'true'


def eq_to_maple(ring: DifferentialRing, eq: sp.Expr, trans_table: Literal['ver1', 'ver2'] = 'ver1') -> str:
    if eq.is_Number:
        return str(eq)
    if eq.is_Symbol or eq.is_Function:
        if trans_table == 'ver1':
            return ring._name_dict[eq.name]
        else:
            return ring._name_dict_2[eq.name]
    if eq.is_Add:
        return ' + '.join(['(' + eq_to_maple(ring, arg, trans_table) + ')' for arg in eq.args])
    if eq.is_Mul:
        return ' * '.join(['(' + eq_to_maple(ring, arg, trans_table) + ')' for arg in eq.args])
    if eq.is_Pow:
        return f'({eq_to_maple(ring, eq.base, trans_table)})^({eq_to_maple(ring, eq.exp, trans_table)})'
    if eq.is_Derivative:
        derivs_arg = eq_to_maple(ring, eq.args[1][0], trans_table)
        if eq.args[1][1] != sp.S.One:
            derivs_arg += '$' + str(eq.args[1][1])
        return f'diff({eq_to_maple(ring, eq.args[0], trans_table)}, {derivs_arg})'
    raise ValueError(f'Error! Parse expression {eq} failed! Func = ', eq.func)

def eq_from_maple(ring: DifferentialRing, eq: str, trans_table: Literal['ver1', 'ver2'] = 'ver1') -> sp.Expr:
    eq = eq.replace('diff', 'Derivative')
    if trans_table == 'ver1':
        for key, value in ring._name_dict.items():
            eq = eq.replace(value, key)
    else:
        for key, value in ring._name_dict_2.items():
            eq = eq.replace(value, key)
    eq = eq.replace('$', ',')
    return sympify(eq)

def gb_from_maple(ring: DifferentialRing, gb: str) -> List[RegularDifferentialChain]:
    if gb == '[]':
        return []
    stack = []
    temp = []
    for i in range(len(gb)):
        if gb[i] in ['[', '(', '{']:
            stack.append(i)
            if gb[i] == '[' and len(stack) == 2:
                temp.append([])
                temp[-1].append('')
        elif gb[i] == ']':
            assert (gb[stack.pop()] == '[')
        elif gb[i] == ')':
            assert (gb[stack.pop()] == '(')
        elif gb[i] == '}':
            assert (gb[stack.pop()] == '{')
        if len(stack) == 2 and gb[i] == ',':
            temp[-1].append('')
        elif len(stack) >= 2 and gb[i] != '[':
            temp[-1][-1] += gb[i]
    try:
        gb = [RegularDifferentialChain(ring, [eq_from_maple(ring, j) for j in i]) for i in temp]
    except:
        print('Error! Failed to parse the output of Maple!')
        print('Output:', temp)
        raise RuntimeError
    return gb
