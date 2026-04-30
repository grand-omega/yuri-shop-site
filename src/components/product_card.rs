use crate::api::catalog::ProductSummary;
use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn ProductCard(product: ProductSummary) -> impl IntoView {
    let ProductSummary {
        slug,
        name,
        subtitle,
        hero_image,
        ..
    } = product;
    let href = format!("/collezione/{}", slug);
    let alt = name.clone();

    view! {
        <A href=href attr:class="group flex flex-col gap-6 text-ink">
            <div class="overflow-hidden bg-ivory">
                <img
                    src=hero_image
                    alt=alt
                    width="1254"
                    height="1254"
                    class="w-full h-auto transition-transform duration-[1400ms] ease-[cubic-bezier(0.7,0,0.3,1)] group-hover:scale-[1.02]"
                />
            </div>
            <div class="flex flex-col gap-2 pt-2">
                <h3 class="font-display text-2xl italic">{name}</h3>
                <p class="font-serif text-graphite/80 text-base">{subtitle}</p>
                <span class="tracker text-ink mt-4 inline-flex items-center gap-3 w-fit">
                    <span>"Su richiesta"</span>
                    <span
                        aria-hidden="true"
                        class="block h-px w-8 bg-ink transition-[width] duration-700 ease-[cubic-bezier(0.7,0,0.3,1)] group-hover:w-16"
                    ></span>
                </span>
            </div>
        </A>
    }
}
