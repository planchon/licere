from auth_types import SchemaGraph, SchemaNode, RelationTuple, Runtime

folder_schema = SchemaGraph()

user_entity = SchemaNode("user", relation={}, permission={})
folder_entity = SchemaNode(
    "folder",
    relation={"parent": "folder", "owner": "user", "reader": "user"},
    permission={
        "read": ["owner", "reader", "parent.read", "parent.edit"],
        "edit": ["owner", "parent.edit"],
    },
)

folder_schema.register_entity(user_entity)
folder_schema.register_entity(folder_entity)

folder_schema.validate_schema()


runtime = Runtime(folder_schema)

tuples = [
    RelationTuple("folder", "root", "owner", "user", "admin"),
    RelationTuple("folder", "france", "parent", "folder", "root"),
    RelationTuple("folder", "dts", "parent", "folder", "france"),
]

for t in tuples:
    runtime.add_tuple(t)

# is a direct match
print(runtime.check("user", "admin", "read", "folder", "root"))

# is not a direct match, should use the parent.read route
print(runtime.check("user", "admin", "read", "folder", "dts"))

# is not a direct match, should use the parent.read route
print(runtime.check("user", "admin", "edit", "folder", "france"))

# is not a direct match, should use the parent.read route
print(runtime.check("user", "paul", "edit", "folder", "france"))
