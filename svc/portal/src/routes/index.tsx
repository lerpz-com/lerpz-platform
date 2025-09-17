import { Index, type VoidComponent } from "solid-js"
import {
  AccordionRoot,
  AccordionContent,
  AccordionItem,
  AccordionTrigger
} from "~/component/ui/accordion"

const Home: VoidComponent = () => {
  return (
    <AccordionRoot multiple>
      <Index each={["React", "Solid", "Vue", "Svelte"]}>
        {(item) => (
          <AccordionItem value={item()}>
            <AccordionTrigger>What is {item()}?</AccordionTrigger>
            <AccordionContent>
              {item()} is a JavaScript library for building user interfaces.
            </AccordionContent>
          </AccordionItem>
        )}
      </Index>
    </AccordionRoot>
  )
}

export default Home
