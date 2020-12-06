interface Identifiable {
  uuid: string;
}

export class Gallery<T extends Identifiable> {
  public items: GalleryItem<T>[];

  constructor() {
    this.items = [];
  }

  append(assets: T[]): Gallery<T> {
    for (let i = 0; i < assets.length; i++) {
      const asset = assets[i];
      if (!this.exists(asset.uuid)) {
        let item = new GalleryItem(asset);
        let prev = this.items[this.items.length - 1] || null;
        if (prev) {
          prev.next = item;
          item.prev = prev;
        }
        this.items.push(item);
      }
    }
    return this;
  }

  prepend(assets: T[]): Gallery<T> {
    for (let i = assets.length - 1; i >= 0; i--) {
      const asset = assets[i];
      if (!this.exists(asset.uuid)) {
        let item = new GalleryItem(asset);
        let next = this.items[0] || null;
        if (next) {
          next.prev = item;
          item.next = next;
        }
        this.items.unshift(item);
      }
    }
    return this;
  }

  exists(uuid: string): boolean {
    return !!this.items.find(item => item.uuid == uuid);
  }

  size(): number {
    return this.items.length;
  }

  isEmpty(): boolean {
    return this.size() == 0;
  }
}

export class GalleryItem<T extends Identifiable> {
  public asset: T;
  public uuid: string;
  public next: GalleryItem<T> | null = null;
  public prev: GalleryItem<T> | null = null;

  constructor(asset: T) {
    this.asset = asset;
    this.uuid = asset.uuid;
  }
}
