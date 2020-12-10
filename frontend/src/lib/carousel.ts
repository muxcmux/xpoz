import type { Asset } from "../codegen/types";
import type { GalleryItem } from "./gallery";
import type { Bounds, Point } from "../utils/math";

export class CarouselItem {
  public item: GalleryItem<Asset> | null;
  public scale: number;
  public width: number;
  public height: number;
  public ratio: number;
  public left: number;
  public top: number;
  public x: number;
  public y: number;
  public jumps: number;
  public pos: number;
  public zoomedScale: number;

  constructor(pos: number) {
    this.item = null;
    this.scale = 1;
    this.width = 0;
    this.height = 0;
    this.ratio = 0;
    this.left = 0;
    this.top = 0;
    this.x = 0;
    this.y = 0;
    this.jumps = 0;
    this.pos = pos;
    this.zoomedScale = 2.7;
  }

  setItem(item: GalleryItem<Asset> | null): CarouselItem {
    this.item = item;
    if (item) {
      const width = item.asset.width;
      const height = item.asset.height;
      const ratio = width / height;
      const desiredHeight = window.innerHeight;
      const desiredWidth  = desiredHeight * ratio;
      if (desiredWidth > window.innerWidth) {
        this.width = window.innerWidth;
        this.height = window.innerWidth / ratio;
        this.top = window.innerHeight / 2 - this.height / 2;
        this.left = 0;
      } else {
        this.height = desiredHeight;
        this.width = desiredWidth;
        this.top = 0;
        this.left = window.innerWidth / 2 - this.width / 2;
      }
      const spacing = 20;
      this.x = this.pos * (window.innerWidth + spacing) + this.jumps * (3 * (window.innerWidth + spacing));
    }
    return this;
  }

  getZoomedBoundsForOrigin(origin: Point): Bounds {
    const deltaX = (this.width * this.zoomedScale - this.width) / 2;
    const deltaY = (this.height * this.zoomedScale - this.height) / 2;
    const minX = origin.x - deltaX;
    const maxX = origin.x + deltaX;
    const minY = origin.y - deltaY;
    const maxY = origin.y + deltaY;
    return { min: { x: minX, y: minY }, max: { x: maxX, y: maxY } };
  }
}

