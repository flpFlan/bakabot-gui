import { reactive } from "vue";
import { Store } from "@tauri-apps/plugin-store";

export const store = new Store(".bakabot-gui.dat");

export class MessageTimer {
  date;
  message;
  groups;
  description;
  sheduled;

  constructor(
    date: Date,
    message: string,
    groups: number[],
    description: string,
    sheduled: boolean = true
  ) {
    this.date = date;
    this.message = message;
    this.groups = groups;
    this.description = description
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
