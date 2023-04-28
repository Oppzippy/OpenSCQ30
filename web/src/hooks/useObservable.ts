import { useEffect, useState } from "react";
import { UnmodifiableBehaviorSubject } from "../UnmodifiableBehaviorSubject";

export function useBehaviorSubject<T>(subject: UnmodifiableBehaviorSubject<T>) {
  const [state, setState] = useState<T>(subject.value);
  useEffect(() => {
    if (subject) {
      const subscription = subject.subscribe((newValue) => setState(newValue));
      return () => subscription.unsubscribe();
    }
  }, [subject]);
  return state;
}
