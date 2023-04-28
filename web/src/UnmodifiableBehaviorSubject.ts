import { BehaviorSubject } from "rxjs";

export type UnmodifiableBehaviorSubject<T> = Omit<
  BehaviorSubject<T>,
  "next" | "complete" | "error"
>;
