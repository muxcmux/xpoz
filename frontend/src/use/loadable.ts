type Variant = "original" | "resized" | "render" | "thumb";
type Params = { uuid: string, variant: Variant };

export default function(node: HTMLImageElement, params: Params) {

  let image: HTMLImageElement | null;
  const { uuid, variant } = params;

  function createLoader(uuid: string, variant: Variant): void {
    destroyLoader();

    const imageServer = "http://192.168.1.2:1234/asset";

    const variants = {
      original: `${imageServer}/original/${uuid}`,
      resized: `${imageServer}/resized/${uuid}`,
      render: `${imageServer}/render/${uuid}`,
      thumb: `${imageServer}/thumb/${uuid}`
    }

    console.log("loading", variant, variants[variant])

    if (variant !== "thumb") node.src = variants.thumb;
    node.classList.remove("loaded", "failed");
    node.classList.add("loading");

    image = new Image();

    image.onload = () => {
      destroyLoader();
      node.classList.add("loaded");
      node.classList.remove("failed", "loading");
      node.src = variants[variant];
    }

    image.onerror = () => {
      destroyLoader();
      node.classList.remove("loaded", "loading");
      node.classList.add("failed");
    }

    image.src = variants[variant];
  }

  function destroyLoader(): void {
    if (image) {
      image.onload = null;
      image.onerror = null;
      image = null;
    }
  }

  createLoader(uuid, variant);

  return {
    update(params: Params) {
      createLoader(uuid, variant);
    },
    destroy() {
      destroyLoader();
    }
  }
}