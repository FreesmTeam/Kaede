import React, { useState } from "react";
import { LaunchMinecraft } from "../../../wailsjs/go/main/App";

export default function HomePage() {
  const [launched, setLaunched] = useState(false);
  const [selectedFile, setSelectedFile] = useState<string>("");
  const [info, setInfo] = useState<string | null>(null);

  function handleFileChange(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0].name || "";
    setSelectedFile(file);
  }

  function launchMinecraft() {
    LaunchMinecraft(selectedFile).then((response) => {
      setLaunched((prev) => !prev);
      setInfo(response.Message);
    });
  }

  return (
    <div>
      <div className="text-white">welcome home</div>
      <img
        alt="Kita oshi no ko dance gif"
        src="https://media.tenor.com/XF0W80UFt-0AAAAj/kita-chan-kitaikuyo.gif"
      />
      <input
        type="file"
        className="rounded-md bg-rose-800 px-2 py-2 m-1"
        accept=".jar"
        onChange={handleFileChange}
      ></input>

      <button
        className="bg-rose-500 py-2 px-4 rounded-md mt-4 active:bg-rose-700 transition"
        onClick={launchMinecraft}
      >
        LAUNCH
      </button>
      {launched && <div className="text-sm text-zinc-300">~ {info}</div>}
    </div>
  );
}
