import json
from typing import Any


class RelationTuple:
    entity_type: str
    entity_id: str
    relation: str
    subject_type: str
    subject_id: str

    def __init__(self, et, ei, r, st, si) -> None:
        self.entity_type = et
        self.entity_id = ei
        self.relation = r
        self.subject_type = st
        self.subject_id = si


class SchemaNode:
    entity_type: str
    relation: dict[str, str]
    permission: dict[str, Any]

    def __init__(
        self, type: str, relation: dict[str, str], permission: dict[str, Any]
    ) -> None:
        self.entity_type = type
        self.relation = relation
        self.permission = permission

    def __str__(self) -> str:
        return f"{self.entity_type}, \nrelations = {json.dumps(self.relation, indent=2)}\npermissions = {json.dumps(self.permission, indent=2)}"


class SchemaGraph:
    nodes: dict[str, SchemaNode]

    def __init__(self) -> None:
        self.nodes = {}

    def register_entity(self, e: SchemaNode) -> None:
        self.nodes[e.entity_type] = e

    def get_node_by_type(self, nt: str):
        return self.nodes[nt]

    def validate_schema(self) -> bool:
        for e in self.nodes.values():
            # check relations
            for k, r in e.relation.items():
                if e.relation[k] not in self.nodes.keys():
                    raise KeyError(
                        f"Entity {e.relation[k]} not found in schema for relation {r}"
                    )

            # check if permissions are ok
            for p in e.permission.values():
                for r in p:
                    if "." in r:
                        relation, permission = r.split(".")
                        if relation not in e.relation.keys():
                            raise KeyError(
                                f"Relation {relation} not in entity {e.entity_type} (for {r})"
                            )
                        continue
                        # should validate that nested type
                    if r not in e.relation.keys():
                        raise KeyError(
                            f"Entity {e.relation[r]} not found in schema for relation {r}"
                        )
        print("Schema valid!")
        return True


class Runtime:
    schema: SchemaGraph
    tuples: list[RelationTuple]

    def __init__(self, schema: SchemaGraph) -> None:
        self.tuples = []
        self.schema = schema

    def add_tuple(self, t: RelationTuple) -> None:
        self.tuples.append(t)

    def find_entity(self, et: str, ei: str) -> RelationTuple | None:
        for t in self.tuples:
            if t.entity_id == ei and t.entity_type == et:
                return t
        return None

    def check(self, et: str, ei: str, perm: str, st: str, si: str) -> bool:
        # basic check (entity & subject & relation exists)
        # find the subject
        ent = self.find_entity(st, si)

        if ent is None:
            print(f"Entity {st} {si} not found, so false")
            return False

        # find the basic relation for the type
        ent_t = self.schema.get_node_by_type(st)
        if perm not in ent_t.permission:
            raise KeyError(f"Permission {perm} not in entity")

        relations = ent_t.permission[perm]

        for r in relations:
            if "." not in r:
                for t in self.tuples:
                    if (
                        t.entity_id == si
                        and t.entity_type == st
                        and t.relation == r
                        and t.subject_id == ei
                        and t.subject_type == et
                    ):
                        print(f"Direct match found {et, ei, r, st, si}")
                        return True
            else:
                # need to do a recursive check
                relation, permission = r.split(".")
                # find tuple implementing the nested relation
                for t in self.tuples:
                    if (
                        t.relation == relation
                        and t.entity_id == si
                        and t.entity_type == st
                    ):
                        print(
                            f"check{et, ei, permission, t.subject_type, t.subject_id}"
                        )
                        if self.check(et, ei, permission, t.subject_type, t.subject_id):
                            return True

        return False
