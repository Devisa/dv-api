pub struct Condition {}
// #[derive(Clone, Eq)]
// pub struct Query {
//     pub id: QueryId,
//     pub prefix: bool,
//     pub exact: bool,
//     pub kind: QueryKind,
// }

// impl PartialEq for Query {
//     fn eq(&self, other: &Self) -> bool {
//         self.prefix == other.prefix && self.kind == other.kind
//     }
// }

// impl Hash for Query {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.prefix.hash(state);
//         self.kind.hash(state);
//     }
// }

// #[derive(Clone, PartialEq, Eq, Hash)]
// pub enum QueryKind {
//     Tolerant(String),
//     NonTolerant(String),
//     Phrase(Vec<String>),
// }

// impl fmt::Debug for Query {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let Query { id, prefix, kind, .. } = self;
//         let prefix = if *prefix { String::from("Prefix") } else { String::default() };
//         match kind {
//             QueryKind::NonTolerant(word) => {
//                 f.debug_struct(&(prefix + "NonTolerant")).field("id", &id).field("word", &word).finish()
//             },
//             QueryKind::Tolerant(word) => {
//                 f.debug_struct(&(prefix + "Tolerant")).field("id", &id).field("word", &word).finish()
//             },
//             QueryKind::Phrase(words) => {
//                 f.debug_struct(&(prefix + "Phrase")).field("id", &id).field("words", &words).finish()
//             },
//         }
//     }
// }

// fn create_operation<I, F>(iter: I, f: F) -> Operation
// where I: IntoIterator<Item=Operation>,
//       F: Fn(Vec<Operation>) -> Operation,
// {
//     let mut iter = iter.into_iter();
//     match (iter.next(), iter.next()) {
//         (Some(first), None) => first,
//         (first, second) => f(first.into_iter().chain(second).chain(iter).collect()),
//     }
// }
