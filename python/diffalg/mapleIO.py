import subprocess

class mapleIO:
    def __init__(self):
        self.commands = []
        self.outputs = []

    def import_lib(self, libname: str):
        """
        Some interesting packages:
          ['DifferentialAlgebra', 'DEtools', 'PDEtools', 'Ore_algebra', 'Physics']
        After import DifferentialAlgebra, we can import Tools to use more functions
        """
        self.commands.append(f'with ({libname})')

    def append_command(self, command: str):
        assert '\n' not in command
        self.commands.append(command)

    def exec_maple(self, timeout: int = 30):
        exec_cmd = '/opt/maple2024/bin/maple'
        func_cmd = ';\n'.join(self.commands)
        exec_args = f'interface(prettyprint=0):\n{func_cmd};'
        process = subprocess.Popen(exec_cmd,
                                   stdin=subprocess.PIPE,
                                   stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE,
                                   encoding='utf-8')
        stdout_, stderr_ = process.communicate(exec_args, timeout=timeout)
        if stderr_:
            raise Exception(stderr_)
        self.outputs = self.translate(stdout_)
        if 'error' in self.outputs[-1].lower():
            print('debug' + '-'*20)
            for c in self.commands:
                print(c)
            print('debug' + '-'*20)
            raise Exception(self.outputs[-1])
        return self.outputs

    def translate(self, stdout: str):
        lines = stdout.split('\n> ')[2:-1]
        outputs = []
        for i, c in enumerate(self.commands):
            temp = lines[i].split(';\n')
            assert temp[0] == c
            assert len(temp) == 2
            outputs.append(temp[1].replace('\n', ' ').replace('\\', ' ').replace(' ', ''))
        return outputs
