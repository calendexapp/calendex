// event_form.ts
import { writable } from 'svelte/store';

export const event_fields = writable({
  summary: "",
  description: "",
  location: "",
  uid: "",
  starts: "",
  ends: ""
})


