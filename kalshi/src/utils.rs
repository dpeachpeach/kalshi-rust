// MACROS

#[macro_export]
macro_rules! add_param {
    ($params:ident, $param_name:expr, $param_value:expr) => {
        if let Some(param) = $param_value {
            $params.push(($param_name, param.to_string()));
        }
    };
}
