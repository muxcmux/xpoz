import Hammer from "hammerjs";

export default function(node: HTMLElement) {
  let hammer = new Hammer.Manager(node, {
    touchAction: "auto",
    recognizers: [
      [Hammer.Tap, { event: "tap" }],
    ]
  });

  hammer.on('singletap', e => {
    node.dispatchEvent(new CustomEvent("singletap", {
      detail: e
    }));
  });

  return {
    destroy() {
      hammer.destroy();
    }
  }
}
