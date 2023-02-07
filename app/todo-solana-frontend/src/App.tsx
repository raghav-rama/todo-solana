import * as React from "react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import "./App.css";

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <nav>
          <h1>
            TodoList {"App "}
            <span>
              powered by{" "}
              <img
                src={require("./assets/img/solana.ico")}
                width={10}
                height={10}
                alt="Solana"
              />
            </span>
            <WalletMultiButton />
          </h1>
        </nav>
      </header>
    </div>
  );
}

export default App;
