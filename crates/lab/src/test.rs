use web_sys::Event;

macro_rules! demo {
    (
    $($(#$($a_cap:ident),+)? $(#)?{ $($($a_name:ident)-+ $(= $a_value:expr)?)*  })?
    $($(*$($s_cap:ident),+)? $(*)?{ $($($s_name:ident)-+: $s_value:expr);* $(;)? })?
    $(@$type:ident = $($e_cap:ident),+ => $b:expr;)*
    $(|$($v_cap:ident),+|  $content:expr)?
    ) => {
        {
            let __tag__ = rtml::tags::p(());

            $(
                let __tag__ = __tag__.attr(rtml::attr! { $($($a_cap),+   #> )? $($($a_name)-+ $(=$a_value)?),*});
            )?
            $(
                let __tag__ = __tag__.style(rtml::style_! { $($($s_cap),+ ~>)? $($($s_name)-+ :$s_value);*});
            )?

            $(
                let __tag__ = __tag__.children(rtml::subs!($($v_cap),+ => $content));
            )?
            $(
                let __tag__ = __tag__.on(stringify!($type), rtml::update!($($e_cap),+ => $b));
            )*
            __tag__
        }
    };
    (
    $($(#$($a_cap:ident),+)? $(#)?{ $($($a_name:ident)-+ $(= $a_value:expr)?)*  })?
    $($(*$($s_cap:ident),+)? $(*)?{ $($($s_name:ident)-+: $s_value:expr);* $(;)? })?
    $(@$type:ident = $($e_cap:ident),+ => $b:expr;)*
    $(|| $content:expr)?
    ) => {
        {
            let __tag__ = rtml::tags::p(());

            $(
                let __tag__ = __tag__.attr(rtml::attr! { $($($a_cap),+   #> )? $($($a_name)-+ $(=$a_value)?),*});
            )?
            $(
                let __tag__ = __tag__.style(rtml::style_! { $($($s_cap),+ ~>)? $($($s_name)-+ :$s_value);*});
            )?

            $(
                let __tag__ = __tag__.children($content);
            )?
            $(
                let __tag__ = __tag__.on(stringify!($type), rtml::update!($($e_cap),+ => $b));
            )*
            __tag__
        }
    };
}

#[test]
fn macro_test() {
    use rtml::IntoReactive;
    use rtml::{input, p, span};

    let count = 0.reactive();

    let d = demo! {
        #count { value=count.val() id="test" pad="xxx" }
        *{ background-color: "red" }
        // TODO 允许不捕获变量
        @click = count => |_| {
            *count.val_mut() += 1;
            true
        };
        |count| format!("count is {}", count.val())
    };
    let _x = input! {
        // # 代表除 style 之外的属性
        #{ id="ok" class="good" }
        // ~ 代表样式
        *{ background-color:"red" }
    };
    let _x = input! {
        // # 后面可选要捕获的响应式数据
        #count { value=count.val() id="test" pad="xxx" }
        // ~ 同理
        *count { background-color: "red" }
        // @ 加上事件类型用来绑定事件函数
        @click = count => |_| {
            *count.val_mut()+=1;
            true
        };
    };
}
