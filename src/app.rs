use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="container mx-auto p-4">
            <h1 class="text-4xl font-bold text-center text-green-600">"Экологический Маркетплейс"</h1>
            <p class="mt-4 text-lg text-gray-700">"Добро пожаловать в мир экологически чистых продуктов!"</p>
            <div class="mt-8 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                <div class="p-4 border rounded-lg shadow-lg">
                    <h2 class="text-2xl font-semibold">"Местные Фермеры"</h2>
                    <p class="mt-2">"Найдите продукты напрямую от местных фермеров в вашем городе."</p>
                </div>
                <div class="p-4 border rounded-lg shadow-lg">
                    <h2 class="text-2xl font-semibold">"Эко Продукты"</h2>
                    <p class="mt-2">"Покупайте экологически чистые продукты, которые полезны для вас и окружающей среды."</p>
                </div>
                <div class="p-4 border rounded-lg shadow-lg">
                    <h2 class="text-2xl font-semibold">"Поддержка Сообщества"</h2>
                    <p class="mt-2">"Поддержите свое местное сообщество, покупая у местных производителей."</p>
                </div>
            </div>
        </div>
    }
}