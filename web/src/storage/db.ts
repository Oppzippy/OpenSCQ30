import Dexie, { Table } from "dexie";

export interface CustomEqualizerProfile {
  id?: number;
  name: string;
  values: number[];
}

export class OpenSCQ30Dexie extends Dexie {
  public customEqualizerProfiles!: Table<CustomEqualizerProfile>;

  public constructor() {
    super("openscq30");
    this.version(1).stores({
      customEqualizerProfiles: "++id, &name, &values",
    });
  }
}

export const db = new OpenSCQ30Dexie();
