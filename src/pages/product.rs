use crate::api::catalog::{get_product, ProductDetail};
use crate::components::{Footer, Header, InquiryForm, SpecItem};
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

#[component]
pub fn ProductPage() -> impl IntoView {
    let params = use_params_map();
    let product = Resource::new(
        move || params.read().get("slug").unwrap_or_default(),
        |slug| async move { get_product(slug).await },
    );

    view! {
        <Header/>
        <main class="pt-[122px] lg:pt-[81px] flex-1">
            <Suspense fallback=move || view! {
                <p class="tracker text-stone text-center py-32">"Loading…"</p>
            }>
                {move || product.get().map(|res| match res {
                    Ok(Some(p)) => view! { <ProductBody product=p/> }.into_any(),
                    Ok(None) => view! {
                        <section class="bg-ink py-32">
                            <p class="tracker text-stone text-center">"Product not found."</p>
                        </section>
                    }.into_any(),
                    Err(e) => view! {
                        <section class="bg-ink py-32">
                            <p class="tracker text-oxblood text-center">{e.to_string()}</p>
                        </section>
                    }.into_any(),
                })}
            </Suspense>
            <Footer/>
        </main>
    }
}

#[component]
fn ProductBody(product: ProductDetail) -> impl IntoView {
    let ProductDetail {
        slug,
        name,
        subtitle,
        kind,
        description_md,
        hero_image,
        specs,
        ..
    } = product;
    let alt = name.clone();
    let has_specs = !specs.is_empty();

    let specs_view = has_specs.then(|| {
        view! {
            <dl class="grid grid-cols-1 sm:grid-cols-2 border-t border-rule bg-ink">
                {specs.into_iter().map(|s| view! { <SpecItem spec=s/> }).collect_view()}
            </dl>
        }
    });

    let cta = match kind.as_str() {
        "bespoke" => view! {
            <div class="flex flex-col gap-6 mt-4">
                <p class="tracker text-stone">"On request · forged for you"</p>
                <InquiryForm product_slug=slug/>
            </div>
        }
        .into_any(),
        _ => view! {
            <p class="tracker text-stone mt-4">"Coming soon."</p>
        }
        .into_any(),
    };

    view! {
        <section class="bg-ink">
            <div class="mx-auto max-w-[1480px] px-6 md:px-8 py-20 lg:py-32 grid grid-cols-1 lg:grid-cols-12 gap-y-16 lg:gap-x-16 items-start">
                <figure class="lg:col-span-7">
                    <img
                        src=hero_image
                        alt=alt
                        width="1254"
                        height="1254"
                        class="w-full h-auto"
                    />
                </figure>
                <div class="lg:col-span-5 flex flex-col gap-10">
                    <div class="flex flex-col gap-4">
                        <p class="tracker text-stone">"Atelier · Verona"</p>
                        <h1 class="font-display font-light text-[clamp(2.5rem,5vw,5rem)] leading-[1.05] tracking-[-0.012em] text-ivory">
                            {name}
                        </h1>
                        <p class="font-serif text-stone text-lg">{subtitle}</p>
                    </div>

                    <p class="font-serif text-ivory text-[1.0625rem] leading-[1.7] whitespace-pre-line max-w-[44ch]">
                        {description_md}
                    </p>

                    <div class="h-px w-28 bg-oxblood"></div>

                    {cta}
                </div>
            </div>

            {specs_view.map(|sv| view! {
                <div class="mx-auto max-w-[1480px] px-6 md:px-8 pb-32 lg:pb-44">
                    {sv}
                </div>
            })}
        </section>
    }
}
