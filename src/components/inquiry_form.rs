use crate::api::inquiry::SubmitInquiry;
use leptos::form::ActionForm;
use leptos::prelude::*;

#[component]
pub fn InquiryForm(#[prop(into, optional)] product_slug: Option<String>) -> impl IntoView {
    let action = ServerAction::<SubmitInquiry>::new();
    let pending = action.pending();
    let value = action.value();

    let hidden_slug = product_slug.map(|slug| {
        view! { <input type="hidden" name="product_slug" value=slug/> }
    });

    view! {
        <ActionForm action=action attr:class="flex flex-col gap-6">
            {hidden_slug}
            <label class="flex flex-col gap-2">
                <span class="tracker text-stone">"Name"</span>
                <input
                    type="text"
                    name="name"
                    required
                    class="bg-transparent border-b border-rule focus:border-ivory outline-none py-2 text-ivory font-serif text-lg transition-colors duration-500"
                />
            </label>
            <label class="flex flex-col gap-2">
                <span class="tracker text-stone">"Email address"</span>
                <input
                    type="email"
                    name="email"
                    required
                    class="bg-transparent border-b border-rule focus:border-ivory outline-none py-2 text-ivory font-serif text-lg transition-colors duration-500"
                />
            </label>
            <label class="flex flex-col gap-2">
                <span class="tracker text-stone">"Message"</span>
                <textarea
                    name="message"
                    rows="5"
                    required
                    class="bg-transparent border-b border-rule focus:border-ivory outline-none py-2 text-ivory font-serif text-lg transition-colors duration-500 resize-none"
                ></textarea>
            </label>
            <button
                type="submit"
                disabled=move || pending.get()
                class="tracker text-ivory pb-3 hover:text-oxblood transition-colors duration-500 text-left disabled:opacity-50 self-start"
            >
                {move || if pending.get() { "Sending…" } else { "Send inquiry →" }}
            </button>
            {move || match value.get() {
                Some(Ok(())) => view! {
                    <p class="tracker text-stone">
                        "Thank you. We'll write from the atelier shortly."
                    </p>
                }.into_any(),
                Some(Err(e)) => view! {
                    <p class="tracker text-oxblood">{e.to_string()}</p>
                }.into_any(),
                None => ().into_any(),
            }}
        </ActionForm>
    }
}
