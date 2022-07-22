<script>
    import {invoke} from "@tauri-apps/api"

    let isConnect = false
    let buttonName = "Подключить VPN"
    const callRust = () => {
        isConnect = isConnect === false;
        invoke('call_rust', {isConnect: isConnect}).then((rustMsg) => {
            rustMsg ? isConnectString = "Подключено" : isConnectString = "Не подключено"
            rustMsg ? buttonName = "Отключить VPN" : buttonName = "Включить  VPN"
        });
    };

    let isConnectString = "Не подключено"

</script>

<main>
    <h1>Добро пожаловать в лучший VPN. </h1>
    <h2>Статус подключения - {isConnectString}</h2>
    <button on:click={callRust}>{buttonName}</button>
</main>

<style>
    h1, h2 {
        margin: 0;
        padding: 0;
        text-decoration: none;
        list-style-type: none;
    }
</style>
