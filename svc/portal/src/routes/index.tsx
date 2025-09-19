import { Index, type VoidComponent } from "solid-js"
import {
  AccordionContent,
  AccordionItem,
  AccordionRoot,
  AccordionTrigger
} from "~/component/ui/accordion"
import {
  Checkbox,
  CheckboxControl,
  CheckboxLabel
} from "~/component/ui/checkbox"

const Home: VoidComponent = () => {
  return (
    <AccordionRoot multiple>
      <Index each={["React", "Solid", "Vue", "Svelte"]}>
        {(item) => (
          <AccordionItem value={item()} disabled={item() === "Vue"}>
            <AccordionTrigger>What is {item()}?</AccordionTrigger>
            <AccordionContent>
              {item()} is a JavaScript library for building user interfaces.
              <Checkbox>
                <CheckboxControl />
                <CheckboxLabel>Check me</CheckboxLabel>
              </Checkbox>
            </AccordionContent>
          </AccordionItem>
        )}
      </Index>
    </AccordionRoot>
  )
}

export default Home
