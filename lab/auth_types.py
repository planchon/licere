import json
from typing import Literal


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


BinaryPermissionOperator = Literal["Or", "And", "Not"]

type Permission = dict[BinaryPermissionOperator, list[str | Permission]]


class SchemaNode:
    entity_type: str
    relation: dict[str, str]
    permission: dict[str, Permission]

    def __init__(
        self,
        type: str,
        relation: dict[str, str],
        permission: dict[str, Permission],
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

    # flatten the object to find all permissions to check
    def find_all_relation_to_check(self, perm: Permission) -> list[str]:
        local_data = []

        def recursive_descend(data):
            for d in data:
                # last level
                if isinstance(d, str):
                    local_data.append(d)
                # have another operator
                if isinstance(d, dict):
                    key = list(d.keys())
                    assert len(key) == 1
                    recursive_descend(d[key[0]])  # type: ignore

        root_key = list(perm.keys())
        assert len(root_key) == 1

        recursive_descend(perm[root_key[0]])  # type: ignore

        return local_data

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
                flat = self.find_all_relation_to_check(p)
                for rr in flat:
                    if "." in rr:
                        relation, _ = rr.split(".")
                        if relation not in e.relation.keys():
                            raise KeyError(
                                f"Relation not found in entity definition {rr}"
                            )
                    else:
                        if rr not in e.relation.keys():
                            raise KeyError(
                                f"Relation not found in entity definition {rr}"
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
        print(f"\nChecking {et, ei, perm, st, si}")
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

        perms = ent_t.permission[perm]

        all_relation_to_check = self.schema.find_all_relation_to_check(perms)

        results = {}

        for r in all_relation_to_check:
            if "." not in r:
                tmp_result = False
                for t in self.tuples:
                    if (
                        t.entity_id == si
                        and t.entity_type == st
                        and t.relation == r
                        and t.subject_id == ei
                        and t.subject_type == et
                    ):
                        print(f"Direct match found {et, ei, r, st, si}")
                        results[r] = True
                        tmp_result = True
                        break
                if tmp_result:
                    continue
            else:
                # need to do a recursive check
                relation, permission = r.split(".")
                # find tuple implementing the nested relation
                tmp_result = False
                for t in self.tuples:
                    if (
                        t.relation == relation
                        and t.entity_id == si
                        and t.entity_type == st
                    ):
                        if self.check(et, ei, permission, t.subject_type, t.subject_id):
                            results[r] = True
                            tmp_result = True
                            break

                if tmp_result:
                    continue

            results[r] = False

        binary_res = self.binary_expression_eval(perms, results)
        print(f"perms eval {perms}")
        print(f"flat results {results}")
        print(f"res {et, ei, perm, st, si} = {binary_res}\n")

        return binary_res

    def binary_expression_eval(self, permission: Permission, flat_result) -> bool:
        top_op = list(permission.keys())[0]

        # do the binary operation evaluation
        def op_eval(op: BinaryPermissionOperator, perms: list[str | Permission]):
            match op:
                case "Or":
                    res = False
                    for p in perms:
                        if isinstance(p, str):
                            res = res or flat_result[p]
                        else:
                            op = list(p.keys())[0]
                            res = res or op_eval(op, p[op])
                    return res
                case "And":
                    res = True
                    for p in perms:
                        if isinstance(p, str):
                            res = res and flat_result[p]
                        else:
                            op = list(p.keys())[0]
                            res = res and op_eval(op, p[op])
                    return res
                case "Not":
                    p = perms[0]
                    if isinstance(p, str):
                        return not flat_result[p]
                    else:
                        op = list(p.keys())[0]
                        return not op_eval(op, p[op])

        return op_eval(top_op, permission[top_op])
