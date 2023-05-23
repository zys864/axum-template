#[macro_export]
macro_rules! model {
    (
        $(#[$m:meta])*
        $vis:vis struct $name:ident {
            $($field_vis:vis $filed:ident : $field_type:ty),* $(,)?
        }
    ) => {
        $(#[$m])*
        $vis struct $name {
            $($field_vis $filed: $field_type),*
        }
    };
    (   $tag:literal,
        $(#[$m:meta])*
        $vis:vis struct $name:ident {
            $($field_vis:vis $filed:ident : $field_type:ty),* $(,)?
        }
    ) => {
        #[derive(serde_tag::WithTag)]
        #[tag(tag = $tag)]
        $(#[$m])*
        $vis struct $name {
            $($field_vis $filed: $field_type),*
        }
    };
}
// model!(
//     "user",
//     pub struct User {
//         pub name: String,
//         pub age: u8,
//     }
// );
#[cfg(test)]
mod tests {
    use super::*;

    

    #[test]
    fn test_name() {

        // let m = User
    }
}
