import { redirect, type Handle } from "@sveltejs/kit"

export const handle: Handle = async ({ event, resolve }) => {
  event.setHeaders({
    "x-clacks-overhead": "GNU Terry Pratchett"
  })

  const res = await resolve(event)
  return res
}