from lab.python.auth_types import SchemaGraph, SchemaNode, RelationTuple, Runtime

folder_schema = SchemaGraph()

user_entity = SchemaNode("user", relation={}, permission={})
folder_entity = SchemaNode(
    "folder",
    relation={"parent": "folder", "owner": "user", "reader": "user", "banned": "user"},
    permission={
        "read": {
            "And": [
                {"Or": ["owner", "reader", "parent.read", "parent.edit"]},
                {"Not": ["banned"]},
            ]
        },
        "edit": {"And": [{"Or": ["owner", "parent.edit"]}, {"Not": ["banned"]}]},
    },
)

folder_schema.register_entity(user_entity)
folder_schema.register_entity(folder_entity)

folder_schema.validate_schema()


runtime = Runtime(folder_schema)

tuples = [
    RelationTuple("folder", "root", "owner", "user", "admin"),
    RelationTuple("folder", "root", "reader", "user", "ppl"),
    RelationTuple("folder", "france", "parent", "folder", "root"),
    RelationTuple("folder", "france", "banned", "user", "ppl"),
    RelationTuple("folder", "dts", "parent", "folder", "france"),
]

for t in tuples:
    runtime.add_tuple(t)

# is a direct match
# print(runtime.check("user", "admin", "read", "folder", "root"))

# is not a direct match, should use the parent.read route
# print(runtime.check("user", "admin", "read", "folder", "dts"))

# is not a direct match, should use the parent.read route
print(runtime.check("user", "admin", "edit", "folder", "france"))

# is not a direct match, should use the parent.read route
print(runtime.check("user", "ppl", "read", "folder", "france"))
