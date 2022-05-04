/// helper macro to create css style
///
/// ```no_run
/// let s = style! {
///     background-color: "#fffff";
///     bar: "bxx";
///     font-size: 100;
///     escaped: r#""abc""#
/// };
/// ```
/// generated
/// ```css
/// background-color: #fffff;
/// bar: bxx;
/// font-size: 100;
/// escaped: "abc";
/// ```
#[macro_export]
macro_rules! style {
    ($($($name:ident)-+: $value:expr);*) => {
        {
            let mut map: HashMap<String, String> = HashMap::new();
            $(
                let name = vec![$(stringify!($name)),*];
                map.insert(name.join("-"), $value.to_string());
            )+
            map
        }
    };
}
