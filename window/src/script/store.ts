import { reactive } from "vue";
import { Store } from "@tauri-apps/plugin-store";

export const store = new Store(".bakabot-gui.dat");

export class MessageTimer {
  date;
  message;
  groups;
  sheduled;

  constructor(
    date: Date,
    message: string,
    groups: number[],
    sheduled: boolean = true
  ) {
    this.date = date;
    this.message = message;
    this.groups = groups;
    this.sheduled = sheduled;
  }
}

export const messageTimerStore: MessageTimer[] = reactive([]);

export async function loadStore() {
  store.get<MessageTimer[]>("messageTimerStore").then((value) => {
    if (value) {
      messageTimerStore.length = 0;
      messageTimerStore.push(...value);
    }
  });
}
