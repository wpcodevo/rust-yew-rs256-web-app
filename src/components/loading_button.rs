use super::spinner::Spinner;
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or(false)]
    pub loading: bool,
    #[prop_or("bg-ct-yellow-600".to_string())]
    pub btn_color: String,
    #[prop_or("text-white".to_string())]
    pub text_color: String,
    pub children: Children,
}

#[function_component(LoadingButton)]
pub fn loading_button_component(props: &Props) -> Html {
    html! {
    <button
      type="submit"
      class={format!(
        "w-full py-3 font-semibold rounded-lg outline-none border-none flex justify-center {}",
         if props.loading {"bg-[#ccc]"} else {props.btn_color.as_str()}
      )}
    >
      if props.loading {
        <div class="flex items-center gap-3">
          <Spinner />
          <span class="text-slate-500 inline-block">{"Loading..."}</span>
        </div>
      }else{
        <span class={props.text_color.clone()}>{props.children.clone()}</span>
      }
    </button>
    }
}
