macro_rules! demo {
    (
    $($(#$($a_cap:ident),+)? $(#)?{ $($($a_name:ident)-+ $(= $a_value:expr)?)*  })?
    $($(*$($s_cap:ident),+)? $(*)?{ $($($s_name:ident)-+: $s_value:expr);* $(;)? })?
    $(@$type:ident = $($e_cap:ident),+ => $b:expr)*
    $(=> $($child:expr),+)?
    ) => {
        {
        let __tag__ = rtml::tags::input(());

        $(
            let __tag__ = __tag__.attr(rtml::attr! { $($($a_cap),+   #> )? $($($a_name)-+ $(=$a_value)?),*});
        )?
        $(
            let __tag__ = __tag__.style(rtml::style_! { $($($s_cap),+ ~>)? $($($s_name)-+ :$s_value);*});
        )?

        $(
            let __tag__ = __tag__.children([$(::std::boxed::Box::new($child) as ::std::boxed::Box<dyn rtml::Template>),+]);
        )?
        $(
            let __tag__ = __tag__.on(stringify!($type), rtml::update!($($e_cap),+ => $b));
        )*
        __tag__
    }};
}

#[test]
fn macro_test() {
    use rtml::IntoReactive;
    use rtml::{input, p, span};

    let count = 0.reactive();

    let d = demo! {
        #count { value=count.val() id="test" pad="xxx" }
        *count  { background-color: "red" }
        @click = count => |_| {
            *count.val_mut() += 1;
            true
        }
        => span!{}, p!{}
    };
    let _x = input! {
        // # 代表除 style 之外的属性
        #{ id="ok" class="good" }
        // ~ 代表样式
        ~{ background-color:"red" }
    };
    let _x = input! {
        // # 后面可选要捕获的响应式数据
        #count { value=count.val() id="test" pad="xxx" }
        // ~ 同理
        ~count { background-color: "red" }
        // @ 加上事件类型用来绑定事件函数
        @click = count => |_| {
            *count.val_mut()+=1;
            true
        }
    };
}
