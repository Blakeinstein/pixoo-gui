import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { WindowControls, WindowTitlebar } from "@tauri-controls/solid";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name: name() }));
  }

  return (
    <div class="flex flex-col w-full h-full">
      <WindowTitlebar>
        PIXOO GUI
      </WindowTitlebar>
      <div class="grow grid place-items-center">
        <textarea class="w-full h-96 text-black" />
      </div>
    </div>
  );
}

export default App;
