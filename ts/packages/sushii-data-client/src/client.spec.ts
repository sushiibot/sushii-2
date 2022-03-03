import ApiClient from "./client";

describe("Client", () => {
  let client: ApiClient;

  beforeAll(async () => {
    client = new ApiClient("http://localhost:3000");
  });

  it("should be defined", () => {
    expect(client).toBeDefined();
  });

  it("should fetch default guild config if not found", () => {
    return expect(client.getGuildConfig("1234")).resolves.toEqual(
      expect.objectContaining({
        id: "1234",
      })
    );
  });
});
