use yew::{function_component, html, Html};

mod props;
use props::Props;

use crate::components::*;

#[function_component]
pub fn ClientReviewCard(props: &Props) -> Html {
    let Props {
        body,
        name,
        company,
    } = props;
    html! {
    <div class="pt-10 text-left rounded-xl bg-gradient-to-r from-teal-900/30 to-purple-900/30 antialiased">
        <figure class="flex px-5">
            <img class="object-cover w-12 h-12 rounded-full" src="images/logo/square/512.png" width="600" height="800" />
            <figcaption class=" pl-6">
            <Label>
            <h3 class="text-left">
            {
                name
            }
            </h3>
            </Label>
            <Label>
            <h3 class="text-left">
            {"CEO of Apple"}
            </h3>
            </Label>

            </figcaption>

        </figure>

        <blockquote class="p-4">
            <Label>
            <h3 class="text-left font-thin">
            
                {body}
            
            </h3>
            </Label>
        </blockquote>

    </div>
    }
}
