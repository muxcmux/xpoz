import Hammer from "hammerjs";

export default function(node: HTMLElement) {
  let hammer = new Hammer.Manager(node, {
    touchAction: "auto",
    recognizers: [
      [Hammer.Tap, { event: "tap" }],
      [Hammer.Pan, { direction: Hammer.DIRECTION_ALL, threshold: 5 }],
    ]
  });

  hammer.on('singletap', e => {
    node.dispatchEvent(new CustomEvent("singletap", {
      detail: e
    }));
  });

  hammer.on('panstart', e => {
    node.dispatchEvent(new CustomEvent("panstart", {
      detail: e
    }));
  });

  hammer.on('panmove', e => {
    node.dispatchEvent(new CustomEvent("panmove", {
      detail: e
    }));
  });

  hammer.on('panend', e => {
    node.dispatchEvent(new CustomEvent("panend", {
      detail: e
    }));
  });

  hammer.on('pancancel', e => {
    node.dispatchEvent(new CustomEvent("panend", {
      detail: e
    }));
  });

  return {
    destroy() {
      hammer.destroy();
    }
  }
}
