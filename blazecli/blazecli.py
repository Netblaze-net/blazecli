import os
import sys

from jinja2 import Environment, FileSystemLoader
import typer
import shutil

app = typer.Typer()



@app.command()
def init():
    def resource_path(relative_path):
        """ Get absolute path to resource, works for dev and for PyInstaller """
        base_path = getattr(sys, '_MEIPASS', os.path.dirname(os.path.abspath(__file__)))
        return os.path.join(base_path, relative_path)

    org = input("Enter company name:\n")
    region = input("Enter region:\n")
    backend_bucket = input("Enter name of backend bucket:\n")
    base_domain = input("Enter name of the base domain:\n")
    variables = {
        "region": region,
        "backend_bucket": backend_bucket,
        "org": org,
        "base_domain": base_domain,
    }

            # Copy the source directory and its contents to the target folder
    shutil.copytree(resource_path('templates'), os.path.join('.', os.path.basename(org)))


    for root, dirs, files in os.walk(f'./{org}'):
        for file in files:

            filepath = os.path.join(root, file)
            dest_path, _ = os.path.splitext(filepath)

            env = Environment(loader=FileSystemLoader(os.path.dirname(filepath)))
            template = env.get_template(file)
            rendered_template = template.render(**variables)
            with open(dest_path, "w") as f:
                f.write(rendered_template)
            os.remove(filepath)

@app.command()
def goodbye(name: str, formal: bool = False):
    if formal:
        print(f"Goodbye Ms. {name}. Have a good day.")
    else:
        print(f"Bye {name}!")


if __name__ == "__main__":
    app()