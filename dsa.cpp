#include <bits/stdc++.h>
using namespace std;

const int mod = 1e9 + 7;
const int N = 2e5;

int add(int x, int y) {
    return (x += y) >= mod ? x - mod : x;
}

int mul(int x, int y) {
    return 1LL * x * y % mod;
}

void upd(int &x, int y) {
    if ((x += y) >= mod) {
        x -= mod;
    }
}

vector<int> g[N];
map<int, int> sum0[N]; // qid, u1 + u2 + u3 + ...
map<int, int> sum1[N]; // qid, u1d1 + u2d2 + ...
int d[N];
int ans[N];

void dfs2(int u, int p) {
    for (int v : g[u]) {
        if (v == p) {
            continue;
        }
        d[v] = d[u] + 1;
        dfs2(v, u);
    }
}

void dfs(int u, int p) {
    for (int v : g[u]) {
        if (v == p) {
            continue;
        }
        dfs(v, u);
        if (sum0[u].size() < sum0[v].size()) {
            swap(sum0[u], sum0[v]);
            swap(sum1[u], sum1[v]);
        }
        vector<int> keys;
        for (auto kv : sum0[v]) {
            keys.push_back(kv.first);
        }
        for (int i : keys) {
            int x = mul(sum0[u][i], sum1[v][i]);
            int y = mul(sum1[u][i], sum0[v][i]);
            int z = mul(2 * d[u], mul(sum0[u][i], sum0[v][i]));
            upd(ans[i], add(add(x, y), mod - z));
            upd(sum0[u][i], sum0[v][i]);
            upd(sum1[u][i], sum1[v][i]);
        }
    }
}

int main() {
    int n, q;
    cin >> n >> q;
    for (int i = 0; i < n - 1; i++) {
        int u, v;
        scanf("%d %d", &u, &v);
        u--;
        v--;
        g[u].push_back(v);
        g[v].push_back(u);
    }
    dfs2(0, -1);
    for (int i = 0; i < q; i++) {
        int k;
        scanf("%d", &k);
        for (int j = 0; j < k; j++) {
            int u;
            scanf("%d", &u);
            u--;
            sum0[u][i] += u + 1;
            sum1[u][i] += mul(u + 1, d[u]);
        }
    }
    dfs(0, -1);
    for (int i = 0; i < q; i++) {
        printf("%d\n", ans[i]);
    }
}