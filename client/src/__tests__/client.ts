import { DefaultApi } from "../api";

describe("API integration test", () => {
  const apiClient = new DefaultApi();
  test("should list users", async () => {
    const response = await apiClient.listUsers();
    expect(response.status).toBe(200);
    expect(response.data.users).toEqual([]);
  });
});
