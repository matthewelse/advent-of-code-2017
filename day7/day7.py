
def parse_root(r):
    root, weight = tuple(r.rstrip(')').split(' ('))
    return root, int(weight)

def parse_line(l):
    parts = l.split(' -> ')

    root, w = parse_root(parts[0].strip())

    if len(parts) == 1:
        return root, w, []
    else:
        children = parts[1].strip().split(', ')
        return root, w, children

def find_wrong(tree, root):
    root_weight, children = tree[root]

    if len(children) == 0:
        return None, { root: root_weight }
    else:
        weights = {}
        child_sums = {}

        cumulative = 0

        for child in children:
            wrong, updates = find_wrong(tree, child)
            weights.update(updates)

            if wrong is not None:
                return wrong, weights

            weight = weights[child]
            cumulative += weight
            if weight not in child_sums:
                child_sums[weight] = { child }
            else:
                child_sums[weight].add(child)

        if len(child_sums) > 1:
            # find the bad child
            # print(child_sums)

            good_weight = 0
            bad_weight = 0

            actual_weight = 0

            for weight, children in child_sums.items():
                if len(children) != 1:
                    good_weight = weight
                else:
                    bad_weight = weight
                    actual_weight = tree[children.pop()][0]

            return actual_weight + (good_weight - bad_weight), weights
        else:
            # print(child_sums)
            weights[root] = cumulative + root_weight
            return None, weights


if __name__ == '__main__':
    with open('day7.in', 'rt') as f:
        lines = f.readlines()

    roots = set()
    inner = set()

    tree = {}

    for l in lines:
        root, w, children = parse_line(l)

        child_set = set(children)
        tree[root] = (w, child_set)
        inner |= child_set

        roots.add(root)

    root = (roots - inner).pop()
    print(root)

    answer = find_wrong(tree, root)
    print(answer[0])

