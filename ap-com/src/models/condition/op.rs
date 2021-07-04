
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Operation {
    And(Vec<Operation>),
    Or(Vec<Operation>),
    Query(Query),
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn pprint_tree(f: &mut fmt::Formatter<'_>, op: &Operation, depth: usize) -> fmt::Result {
            match op {
                Operation::And(children) => {
                    writeln!(f, "{:1$}AND", "", depth * 2)?;
                    children.iter().try_for_each(|c| pprint_tree(f, c, depth + 1))
                },
                Operation::Or(children) => {
                    writeln!(f, "{:1$}OR", "", depth * 2)?;
                    children.iter().try_for_each(|c| pprint_tree(f, c, depth + 1))
                },
                Operation::Query(query) => writeln!(f, "{:2$}{:?}", "", query, depth * 2),
            }
        }

        pprint_tree(f, self, 0)
    }
}

impl Operation {
    fn tolerant(id: QueryId, prefix: bool, s: &str) -> Operation {
        Operation::Query(Query { id, prefix, exact: true, kind: QueryKind::Tolerant(s.to_string()) })
    }

    fn non_tolerant(id: QueryId, prefix: bool, s: &str) -> Operation {
        Operation::Query(Query { id, prefix, exact: true, kind: QueryKind::NonTolerant(s.to_string()) })
    }

    fn phrase2(id: QueryId, prefix: bool, (left, right): (&str, &str)) -> Operation {
        let kind = QueryKind::Phrase(vec![left.to_owned(), right.to_owned()]);
        Operation::Query(Query { id, prefix, exact: true, kind })
    }
}
