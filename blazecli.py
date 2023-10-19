import os
from jinja2 import Environment, FileSystemLoader
import typer

app = typer.Typer()



@app.command()
def init():
    org = input("Enter company name:\n")
    region = input("Enter region:\n")
    backend_bucket = input("Enter name of backend bucket:\n")
    base_domain = input("Enter name of the base domain:\n")


    base_dir = org
    subdirs = ['.github', 'init', 'shared']

    variables = {
        "region": region,
        "backend_bucket": backend_bucket,
        "org": org,
        "base_domain": base_domain,
    }

    os.mkdir(base_dir)
    for dr in subdirs:
        if dr == ".github":
            os.mkdir(os.path.join(base_dir, dr))
            os.mkdir(os.path.join(base_dir, dr, 'workflows'))
        else:
            os.mkdir(os.path.join(base_dir, dr))

    root_directory = './templates'

    for root, dirs, files in os.walk(root_directory):
        for file in files:

            filepath = os.path.join(root, file)
            dest_path, _ = os.path.splitext(filepath.replace("templates", org))

            env = Environment(loader=FileSystemLoader(os.path.dirname(filepath)))
            template = env.get_template(file)
            rendered_template = template.render(**variables)
            with open(dest_path, "w") as f:
                f.write(rendered_template)


@app.command()
def goodbye(name: str, formal: bool = False):
    if formal:
        print(f"Goodbye Ms. {name}. Have a good day.")
    else:
        print(f"Bye {name}!")


if __name__ == "__main__":
    app()