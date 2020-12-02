export const getMyAlbums = `
  query($page: Int!) {
    myAlbums(page: $page) {
      id
      uuid
      title
      photosCount
      videosCount
      createdAt
      keyAssets {
        uuid
      }
    }
  }
`;

export const getAlbum = `
  query($uuid: String!, $offset: Int!, $limit: Int!) {
    album(uuid: $uuid) {
      uuid
      title
      photosCount
      videosCount
      assets(offset: $offset, limit: $limit) {
        uuid
        width
        height
      }
    }
  }
`;
