import numpy as np
import json
from pathlib import Path
from dataclasses import dataclass

@dataclass
class Table:
    x: np.ndarray
    y: np.ndarray

    @staticmethod
    def new(len: int) -> 'Table':
        rng = np.random.default_rng(12345 * len)

        x = rng.uniform(low=0, high=100, size=len)
        y = rng.uniform(low=0, high=10, size=len)
        x.sort()

        return Table(x,y)

@dataclass
class TestCase():
    table: Table
    lookup_inputs: np.ndarray
    lookup_outputs: np.ndarray

@dataclass
class TestCases():
    cases: list[TestCase]

def main():
    clamped_test_cases = []

    for length in [10, 20, 30, 40]:
        table = Table.new(length)

        x_values = np.linspace(-10, 110, 100)
        y_values = np.interp(x_values, table.x, table.y)

        case = TestCase(table, x_values, y_values)
        clamped_test_cases.append(case)

    output_json = {
        "clamped": []
    }

    for case in clamped_test_cases:
        output_dict = {
            "x": list(case.table.x),
            "y": list(case.table.y),
            "input": list(case.lookup_inputs),
            "output": list(case.lookup_outputs),
        }

        output_json["clamped"].append(output_dict);

    with open("tests/lookup_table_1d_cases.json", "w") as f:
        json.dump(output_json, f, indent=1)
        

if __name__ == "__main__":
    main()
