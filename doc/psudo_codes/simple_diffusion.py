def membrane_permeate(a_solvent_o2_count, b_solvent_o2_count, o2_permeability):
    o2_forward = int(round(a_solvent_o2_count * o2_permeability))
    o2_reverse = int(round(b_solvent_o2_count * o2_permeability))

    a_solvent_o2_count = a_solvent_o2_count - o2_forward
    b_solvent_o2_count = b_solvent_o2_count - o2_reverse

    a_solvent_o2_count = a_solvent_o2_count + o2_reverse
    b_solvent_o2_count = b_solvent_o2_count + o2_forward

    return a_solvent_o2_count, b_solvent_o2_count


a_solvent_o2_count = 10000
b_solvent_o2_count = 10
o2_permeability = 0.3
for cycle in range(10):
    a_solvent_o2_count, b_solvent_o2_count = membrane_permeate(
        a_solvent_o2_count,
        b_solvent_o2_count,
        o2_permeability,
    )
    print(f"{cycle},{a_solvent_o2_count},{b_solvent_o2_count}")
