import Hammer from "hammerjs";

export default function(node: HTMLElement) {
  let hammer = new Hammer.Manager(node, {
    recognizers: [
      [Hammer.Pinch],
      [Hammer.Pan, { direction: Hammer.DIRECTION_ALL, threshold: 5 }],
      [Hammer.Swipe,{ direction: Hammer.DIRECTION_ALL }],
    ]
  });

  hammer.on('panstart', e => {
    node.dispatchEvent(new CustomEvent("panstart", {
      detail: e
    }))
  })

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