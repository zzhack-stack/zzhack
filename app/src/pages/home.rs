use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    html! {
        <div class="w-full h-full flex justify-center items-center">
            <div class="w-[1063px]">
                <div class="flex gap-[30px]">
                    <div class="w-[287px] h-[251px] rounded-[35px]">
                        <div class="w-full h-full blur-lg bg-[#01A27E]" />
                    </div>
                    <div class="w-[746px] h-[251px] rounded-[35px] bg-[#1A73D2]">

                    </div>
                </div>
                <div class="flex gap-[30px] mt-[30px]">
                    <div class="w-[428px] h-[251px] rounded-[35px] bg-[#FED214]">

                    </div>
                    <div class="w-[287px] h-[251px] rounded-[35px] bg-[#222629]">

                    </div>
                    <div class="w-[287px] h-[251px] rounded-[35px] bg-[#EB9EB2]">

                    </div>
                </div>
            </div>
        </div>
    }
}
