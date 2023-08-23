/* @refresh reload */
import { render } from "solid-js/web";

import "nes.css/css/nes.min.css";
import "./styles.css";
import App from "./App";

render(() => <App />, document.getElementById("root") as HTMLElement);
