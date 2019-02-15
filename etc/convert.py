#!/usr/bin/python3

import json


root = {
    "tree": {}
}

for line in open("api.txt", "r"):
    line = line.strip()
    if len(line) == 0:
        continue
    line = line.split(" ")[0]

    req = False
    if line[-1] == "?":
        req = True
        line = line[:-1]
    seq = line.split(":")

    branch = root
    for name in seq:
        tree = branch["tree"]
        if name not in tree:
            tree[name] = { "tree": {} }
        branch = tree[name]

    branch["cmd"] = True
    if req:
        branch["req"] = True
    else:
        branch["req"] = True

with open("api.json", "w") as file:
    file.write(json.dumps(
        root,
        sort_keys=True,
        indent=2,
    ))

def gen_method(prefix, name):
    get_fn = """
    pub fn {name}_get(&mut self) -> io::Result<String> {
        self.handle.get({topic})
    }"""
    set_fn = """
    pub fn {name}_set(&mut self, data: &str) -> io::Result<()> {
        self.handle.get({topic}, data)
    }"""


def gen_branch(prefix, name, branch):
    src = """
pub struct {name}<H: Handle> {
    handle: H,{child_fields}
}
impl {name} {
    fn new(handle: H) -> {name} {
        {name} {
            handle,{child_news}
        }
    }{child_methods}
}
"""
    child_src = ""
    children = []
    for child_name, child in branch["tree"].items():
        children.append(child)
    #src.format(name=name,)
    src += "pub struct %s {" % name;
    src += "\t" % name;

with open("api.rs", "w") as file:
    file.write(gen_branch([], "Keysight53220A", root))

