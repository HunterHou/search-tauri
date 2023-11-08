import { WebviewWindow, getAll } from "@tauri-apps/api/window";
import { Command } from "@tauri-apps/api/shell";
import {
  renameFile,
  removeFile,
  exists,
  removeDir,
  readDir,
} from "@tauri-apps/api/fs";

export const NewWindow = async ({ title, url, wid }) => {
  const wins = getAll();
  const indexWindow = wins.findIndex((item) => item.label == wid);
  if (indexWindow >= 0) {
    await wins[indexWindow].close();
  }
  const webview = new WebviewWindow(wid, {
    title: title,
    focus: true,
    skipTaskbar: true,
    width: 1200,
    height: 850,
    url,
  });
  webview.once("tauri://created", function () {
    console.log("tauri://created");
  });
};

export const explorerBySystem = async ({ Path }) => {
  console.log("explorerBySystem", Path);
  const command = new Command("explorer", [Path]);
  command.on("close", (data) => {
    console.log(
      `command finished with code ${data.code} and signal ${data.signal}`
    );
  });
  command.on("error", (error) => console.error(`command error: "${error}"`));
  command.stdout.on("data", (line) => console.log(`command stdout: "${line}"`));
  command.stderr.on("data", (line) => console.log(`command stderr: "${line}"`));
  command.spawn();
};

export const shutdownBySystem = async () => {
  console.log("shutdownBySystem");
  const command = new Command("shutdown", ["-p"]);
  command.on("close", (data) => {
    console.log(
      `command finished with code ${data.code} and signal ${data.signal}`
    );
  });
  command.on("error", (error) => console.error(`command error: "${error}"`));
  command.stdout.on("data", (line) => console.log(`command stdout: "${line}"`));
  command.stderr.on("data", (line) => console.log(`command stderr: "${line}"`));
  command.spawn();
};

export const DeleteDir = async ({ Path }) => {
  const entries = await readDir("users", { dir: Path, recursive: true });
  if (entries.length > 0) {
    for (let i = 0; i < entries.length; i++) {
      const entry = entries[i];
      if (entry.children?.length > 0) {
        await DeleteDir({ Path: entry.path });
      } else {
        await DeleteFile({ Path: entry.path });
      }
    }
  }
  await removeDir(Path);
};

export const DeleteFile = async ({ Path }) => {
  if (await exists(Path)) {
    await removeFile(Path);
  }
};

export const renameFileToDisk = async ({ oldPath, newPath }) => {
  await renameFile(oldPath, newPath);
};

export default {
  NewWindow,
  explorerBySystem,
  shutdownBySystem,
  DeleteDir,
  DeleteFile,
  renameFileToDisk,
};
