import { FileArchive } from "lucide-react";
import { Checkbox } from "@/components/ui/checkbox";

import {} from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import { sendNotification } from '@tauri-apps/api/notification';
import { invoke } from '@tauri-apps/api/tauri';

listen("tauri://file-drop", async (event) => {
  const paths: string[] = event.payload as string[];
  for await (const path of paths) {
    const result = await invoke("unzip_file", { filePath: path });
    if (result) {
      sendNotification({
        title: 'Drop to extract',
        body: `The file ${path} was successfully extracted.`
      });
    } else {
      sendNotification({
        title: 'Drop to extract',
        body: `The file ${path} was successfully extracted.`
      });
    }
  }
});

function App() {
  return (
    <div className="w-screen h-screen bg-background p-4 select-none">
      <div className="w-full h-full bg-foreground flex flex-col items-center justify-center gap-y-3 border-4 border-dashed border-border rounded-md">
        <FileArchive className="text-gray-400" />
        <h2 className="text-gray-400 font-bold text-[18px]">Drop .zip or .rar files</h2>

        <div className="items-top flex space-x-2">
          <Checkbox
          id="terms1"
          onCheckedChange={(e) => {
            invoke('set_config', { cfg: e })
          }} />
          <div className="grid gap-1.5 leading-none">
            <label
              htmlFor="terms1"
              className="text-[12px] font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 text-gray-400"
            >
              Remove the file after the extracted
            </label>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
