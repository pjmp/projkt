import sys
from os import path
import subprocess

code = """use crate::plugins::{{
    types::{{Plugin, ProjktResult}},
}};

pub struct {0}Options;

pub struct {0};

impl Plugin for {0} {{
    type Opts = {0}Options;
    type Fetch = ();

    fn fetch(_: &Self::Opts) -> ProjktResult<Self::Fetch> {{
        unimplemented!()
    }}

    fn exec(_: Self::Opts) -> ProjktResult<()> {{
        unimplemented!()
    }}
}}
"""


def create_plugin(name):
    dest = path.join(
        path.realpath(path.curdir), "src", "plugins")

    if not path.exists(dest):
        print("Invoke this from the root of directory")
        exit(1)

    file_name = f"{path.join(dest, str.lower(name))}.rs"

    with open(file_name, "w") as f:
        name = str.capitalize(name)
        f.write(code.format(name))

    with open(f"{dest}.rs", "a") as f:
        f.write(f"\npub mod {str.lower(name)};")


def main():
    match sys.argv[1:]:
        case []:
            print("Please provide a module name")

        case [plugin_name]:
            create_plugin(plugin_name)

            subprocess.run(["cargo", "fmt"])

        case _:
            print("Too many arguments provided, takes max 1 args.")


if __name__ == "__main__":
    main()
