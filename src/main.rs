use yew::prelude::*;
use web_sys::{window, HtmlInputElement};
use gloo_storage::{LocalStorage, Storage};
use serde::{Serialize, Deserialize};
use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Serialize, Deserialize, Clone)]
struct SaveData {
    name: String,
    players: String,
    roles: String,
    duplicate: bool
}


#[function_component]
fn App() -> Html {
    let saved_setting = use_state(|| {
        LocalStorage::get::<Vec<SaveData>>("role_generator_setting").unwrap_or_default()
    });
    let settings_input = use_state(|| String::new());
    let players_input = use_state(|| String::new());
    let roles_input = use_state(|| String::new());
    let allow_duplicate = use_state(|| false);
    let result_list = use_state(|| None::<Vec<String>>);
    
    let on_input_players = {
        let players_input = players_input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                players_input.set(input.value());
            }
        })
    };

    let on_input_roles = {
        let roles_input = roles_input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                roles_input.set(input.value());
            }
        })
    };

    let on_allow_duplicate = {
        let allow_duplicate = allow_duplicate.clone();
        Callback::from(move |event: Event| {
            let input = event.target_unchecked_into::<HtmlInputElement>();
            allow_duplicate.set(input.checked());
        })
    };

    let on_input_setting = {
        let settings_input = settings_input.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                settings_input.set(input.value());
            }
            
        })
    };

    let load_settings = {
        let saved_setting = saved_setting.clone();
        let settings_input = settings_input.clone();
        let allow_duplicate = allow_duplicate.clone();
        let roles_input = roles_input.clone();
        let players_input = players_input.clone();


        Callback::from(move |_| {
            if !settings_input.is_empty() {
                let items = (*saved_setting).clone();
                if let Some(index) = items.iter().position(|item| item.name == (*settings_input).clone() ) {
                    let item = &items[index];
                    players_input.set(item.players.clone());
                    roles_input.set(item.roles.clone());
                    allow_duplicate.set(item.duplicate);
                } else {
                    if let Some(win) = window(){
                        win.alert_with_message("Сохранения не найдено").expect("");
                    }
                }
            } else {
                if let Some(win) = window(){
                    win.alert_with_message("Введите или выберите имя сохранения").expect("");
                }
            }
        })
    };

    let save_settings = {
        let saved_setting = saved_setting.clone();
        let settings_input = settings_input.clone();
        let allow_duplicate = allow_duplicate.clone();
        let roles_input = roles_input.clone();
        let players_input = players_input.clone();


        Callback::from(move |_| {
            if !settings_input.is_empty() && !roles_input.is_empty() && !players_input.is_empty(){
                let mut items = (*saved_setting).clone();
                if let Some(index) = items.iter().position(|item| item.name == (*settings_input).clone() ) {
                    items.remove(index);
                }
                items.push(SaveData { 
                    name: (*settings_input).clone(), 
                    players: (*players_input).clone(), 
                    roles: (*roles_input).clone(), 
                    duplicate: *allow_duplicate });
                LocalStorage::set("role_generator_setting", &items).expect("Save error");
                saved_setting.set(items);
            } else {
                if let Some(win) = window(){
                    win.alert_with_message("Не все данные были введены").expect("");
                }
            }
        })
    };

    let delete_settings = {
        let saved_setting = saved_setting.clone();
        let settings_input = settings_input.clone();

        Callback::from(move |_| {
            if !settings_input.is_empty(){
                let mut items = (*saved_setting).clone();
                if let Some(index) = items.iter().position(|item| item.name == (*settings_input).clone() ) {
                    items.remove(index);
                }
                LocalStorage::set("role_generator_setting", &items).expect("Save error");
                saved_setting.set(items);
            } else {
                if let Some(win) = window(){
                    win.alert_with_message("Введите или выберите имя сохранения").expect("");
                }
            }
        })
    };

    let generate_result = {
        let result_list = result_list.clone();
        let allow_duplicate = allow_duplicate.clone();
        let roles_input = roles_input.clone();
        let players_input = players_input.clone();
        let mut retern_list = Vec::new();

        let players = match players_input.parse::<u128>() {
            Ok(v) => (1..=v).map(|i| format!("{}", i)).collect::<Vec<String>>(),
            Err(_) => players_input.split_whitespace().map(|v| v.to_string()).collect::<Vec<String>>()
        };

        let mut roles = match roles_input.parse::<u128>() {
            Ok(v) => (1..=v).map(|i| format!("{}", i)).collect::<Vec<String>>(),
            Err(_) => roles_input.split_whitespace().map(|v| v.to_string()).collect::<Vec<String>>()
        };
        let mut rng = thread_rng();
        roles.shuffle(&mut rng);
        if *allow_duplicate {
            retern_list = players.iter().map(|p| format!("{}: {}", p, roles[rng.gen_range(0..roles.len())])).collect::<Vec<String>>();
        } else if roles.len()>=players.len() {
            for i in 0..players.len() {
                retern_list.push(format!("{}: {}", players[i], roles[i]));
            }
        }
        
        Callback::from(move |_| {
            if !retern_list.is_empty() {
                result_list.set(Some(
                    retern_list.clone()
                ));
            } else {
                if let Some(win) = window(){
                    win.alert_with_message("Не удалось сгенерировать, возможно недостаточно ролей, добавьте больше ролей или разрешите дублирование.").expect("");
                }
                result_list.set(None);
            }
        })
    };

    html! {
        <div class="container">
            <h1>{"Генератор ролей"}</h1>
            <p>{"Настройте параметры, введите данные и сгенерируйте роли."}</p>

            <div class="settings">
                <label for="saveName">{"Имя сохранения:"}</label>
                <input id="saveName" list="saveNameList"  value={(*settings_input).clone()} oninput={on_input_setting}/> //value="#" oninput="#"
                <datalist id="saveNameList">
                    { (*saved_setting).clone().into_iter().map(|item| {
                        html!{
                            <option>{item.name}</option>
                        }
                    }).collect::<Html>() }
                </datalist>
                <button class="load" onclick={load_settings}>{"Загрузить"}</button>
                <button class="save" onclick={save_settings}>{"Сохранить"}</button>
                <button class="delete" onclick={delete_settings}>{"Удалить"}</button>
            </div>

            <div class="input-block">
                <div class="input-group">
                    <label for="playersInput" >{"Введите количество игроков или перечислити их через пробел:"}</label>
                    <input type="text" id="playersInput" placeholder="Например: 5 или Иван Мария Сергей..." value={(*players_input).clone()} oninput={on_input_players}/>
                </div>
                <div class="input-group">
                    <label for="rolesInput">{"Введите количество ролей или перечеслити их через пробел:"}</label>
                    <input type="text" id="rolesInput" placeholder="Например: 3 или Миротворец Детектив Стратег..." value={(*roles_input).clone()} oninput={on_input_roles}/>
                </div>
                <div class="checkbox-group">
                    <input type="checkbox" id="duplicateRoles" checked={*allow_duplicate} onchange={on_allow_duplicate}/>
                    <label for="duplicateRoles">{"могут ли роли дублироваться"}</label>
                </div>
                <button class="generate-btn" onclick={generate_result}>{"Сгенерировать"}</button>
            </div>

            <div class="output">
                <h2>{"Результаты:"}</h2>
                    {
                        match (*result_list).clone() {
                            Some(v) => v.into_iter().map(|item| {
                                html!{
                                    <div class="result-item">{item}</div>
                                }
                            }).collect::<Html>(),
                            None => html!{},
                        }
                    }
            </div>
        </div>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}