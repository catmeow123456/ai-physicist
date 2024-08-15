import re
from typing import List, Set, Dict
import numpy as np
import sympy as sp

# 这个文件存储的是纯粹的 sympy 表达式运算相关的函数

# Delta(x(t)) 表示的是一组关于时间的数据 x(t) 相对于 x(t) 平均值的差分。
class Delta(sp.Expr):
    def __new__(cls, expr: str | sp.Expr):
        obj = sp.Expr.__new__(cls, sp.sympify(expr))
        return obj

    def _latex(self, printer):
        x = self.args[0]
        _x = printer._print(x)
        return r'{\Delta\left[ %s \right]}' % (_x)

    # The doit method is used to simplify the expression.
    def doit(self, **hints) -> sp.Expr:
        x = self.args[0]
        if x.is_number:
            return sp.Number(0)
        _x: list[sp.Expr] = x.as_coeff_add()[1]
        y = 0
        for i in _x:
            i1, i2 = i.as_coeff_Mul()
            y += i1 * Delta(i2)
        return y

    def _eval_is_commutative(self):
        return self.args[0].is_commutative


def get_variables(eqlist: List[sp.Expr]) -> Set[sp.Expr]:
    """
    Get the variables in the groebner basis.
    """
    variables = set()
    for eq in eqlist:
        variables |= eq.free_symbols
        variables |= eq.atoms(sp.Derivative)
    return variables

def normalize_factor(expr: sp.Expr):
    """
    Get the normalization factor of an expression,
    which is the max of the absolute values of the coefficients.

    Parameters:
    expr (sp.Expr): The expression to normalize.

    Returns:
    float: The normalization factor.

    """
    temp = sp.expand(expr)
    if issubclass(temp.func, sp.Mul):
        coefficient = temp.as_coeff_mul()[0]
        return coefficient
    else:
        coefficients_dict = sp.expand(expr).as_coefficients_dict()
        factor = sorted(list(coefficients_dict.values()), key=np.abs)[-1]
        return factor


def compact_transform(eq_input: sp.Expr):
    """
    注释待补充QAQ
    """
    eq_factor = sp.factor(sp.expand(eq_input))
    if eq_factor.is_Mul:
        eq_factor = eq_factor.as_ordered_factors()
        eq_factor = sp.Mul(*[compact_transform(eq) for eq in eq_factor])
        return eq_factor
    elif eq_factor.is_number:
        return eq_factor
    exprs = eq_factor.as_ordered_terms()
    # get intrinsic exprs (name+id)
    factor_exprs = {}
    for expr in exprs:
        symbols = expr.atoms(sp.Symbol, sp.Derivative)
        for symbol in symbols:
            quotient = sp.reduced(eq_factor, [symbol])[0][0]
            if symbol not in quotient.free_symbols and len(quotient.as_ordered_terms()) > 1:
                factor_exprs[symbol] = len(quotient.as_ordered_terms())
    factors = sorted(list(factor_exprs.keys()), key=lambda x: factor_exprs[x], reverse=True)
    if factors != []:
        eq = sp.collect(eq_factor, factors)
        return sp.Add(*[sp.factor(term) for term in eq.as_ordered_terms()])
    return eq_factor



def mapping_subscript(expr: sp.Expr, mapping: List | Dict) -> sp.Expr:
    """
    Maps subscripts in an expression to their corresponding values based on the given mapping.
    All possible mappings can be given as a list or a dictionary.
    If a list is given, the mapping is assumed to be in order, i.e., the substitution rule is {1: mapping[0], 2: mapping[1], ...}.
    (This is generally used for converting concepts to an expression.)
    If a dictionary is given, the mapping is directly used for substitution.

    Args:
        expr (sp.Expr): The expression containing variables with subscripts to be mapped.
        mapping (list | dict): The mapping of subscripts. It can be either a list or a dictionary.

    Returns:
        sp.Expr: The expression with subscripts mapped to their corresponding values.

    Raises:
        ValueError: If the mapping is neither a list nor a dictionary.
        ValueError: If the expression contains multiple subscripts.
    """
    variables = expr.free_symbols
    # Create the translation dictionary for the mapping of subscripts
    if isinstance(mapping, list):
        mapping_dict = str.maketrans({str(i+1): str(mapping[i]) for i in range(len(mapping))})
    elif isinstance(mapping, dict):
        mapping_dict = str.maketrans({str(i): str(mapping[i]) for i in mapping})
    else:
        raise ValueError("mapping should be a list or a dictionary")
    # Create a dictionary to store the substitution rules
    var_dict = {}
    for var in variables:
        temp = str(var).split('_')
        if len(temp) <= 1:
            continue
        if len(temp) > 2:
            raise ValueError("can not mapping subscript in an expression with multi subscipts")
        subscript = temp[1].translate(mapping_dict)
        var_dict[var] = sp.Symbol(temp[0] + '_' + subscript)
    return expr.subs(var_dict, simultaneous=True)


def remove_subscript(expr: sp.Symbol) -> sp.Symbol:
    """
    这个函数将形如 MP1_[1,2] 这样的符号去除其下标转换为 MP1
    """
    temp = str(expr).split('_')
    if len(temp) == 1:
        return expr
    elif len(temp) == 2:
        return sp.Symbol(temp[0])
    else:
        raise ValueError("can not remove subscript in an expression with multi subscipts")

def get_subscript(expr: sp.Symbol) -> List[int]:
    """
    这个函数将形如 MP1_[1,2] 这样的符号去除其下标转换为 [1,2]
    """
    temp = str(expr).split('_')
    if len(temp) == 1:
        return []
    elif len(temp) == 2:
        return eval(temp[1])
    else:
        raise ValueError("can not get subscript in an expression with multi subscipts")



"""
sympify a expression
"""

def sympify(expr_str: str):
    try:
        pattern = re.compile(r'([A-Za-z0-9]+_\[[0-9, ]+\])')
        symbols = pattern.findall(expr_str)
        symbols_s = [subscripts_change(symbol) for symbol in symbols]
        trans_rule = {symbol: symbol_s for symbol, symbol_s in zip(symbols, symbols_s)}
        expr = replace_terms_with_dict(expr_str, trans_rule)
        expr = sp.sympify(expr, locals={'Delta': Delta})
        expr = expr.subs({sp.Symbol(symbol_s): sp.Symbol(symbol) for symbol, symbol_s in zip(symbols, symbols_s)})
    except Exception as e:
        raise ValueError(f"Can not sympify the expression: {expr_str}") from e
    return expr

def replace_terms_with_dict(string, replacements):
    # Regular expression to match variables with subscript notation
    pattern = re.compile(r'([A-Za-z0-9]+)_\[([0-9, ]+)\]')

    def replace(match):
        variable, subscripts = match.groups()
        term = f"{variable}_[{subscripts}]"
        return replacements.get(term, term)

    # Replace terms in the string using the dictionary
    return pattern.sub(replace, string)


"""
subscript manipulation
"""

def subscripts_change(symbol_str: str):
    name = symbol_str.split('_')
    if len(name) == 1:
        return symbol_str
    elif len(name) == 2:
        subscripts = eval(name[1])
        name = name[0]+'_'
        if isinstance(subscripts, list):
            for sub in subscripts:
                name = name + 's'+str(sub)
        else:
            name = name + str(subscripts)
        return name
    else:
        raise ValueError('Can not used for symbol with multi _')

def subscript_form_trans(vars: List[sp.Expr]):
    vars_all = []
    transform_dict = {}
    for var in vars:
        new = sp.Symbol(subscripts_change(var.name))
        vars_all.append(new)
        transform_dict.update({var: new})
    return vars_all, transform_dict


def subscript_form_inverse(vars: List[sp.Expr]):
    vars_all = []
    transform_dict = {}
    for var in vars:
        new = sp.Symbol(subscripts_inverse(var.name))
        vars_all.append(new)
        transform_dict.update({var: new})
    return vars_all, transform_dict



def subscripts_inverse(symbol_str: str):
    name = symbol_str.split('_s')
    if len(name) == 1:
        return symbol_str
    elif len(name) == 2:
        subscripts = [int(i) for i in name[1].split('s')]
        name = name[0]+'_' + str(subscripts)
        return name
    else:
        raise ValueError('Can not used for symbol with multi _')




def find_powers(expr, symbol):
    derivs = expr.atoms(sp.Derivative)
    expr_no_deriv = expr.subs({deriv: 1 for deriv in derivs})
    if expr_no_deriv.is_Atom:
        if expr == symbol:
            return [1]
        else:
            return []
    if expr_no_deriv.is_Pow:
        base, exp = expr_no_deriv.as_base_exp()
        if base == symbol:
            return [exp]
    powers = []
    for arg in expr_no_deriv.args:
        powers.extend(find_powers(arg, symbol))
    return powers