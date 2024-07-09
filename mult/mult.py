# # multiplication
# for i in range(1,10):
#     print (f"5x{i} = {5*i}")

# random inventory program
import time

def add_item(inv, item_name, quantity):
    if item_name in inv:
        inv[item_name] += quantity
    else:
        inv[item_name] = quantity

def remove_item(inv, item_name, quantity):
    if item_name in inv:
        if inv[item_name] >= quantity:
            inv[item_name] -= quantity
            if inv[item_name] == 0:
                del inv[item_name]
        else:
            print(f"Not enough {item_name} in inventory.")
    else:
        print(f"{item_name} not found in inventory.")

def display_inv(inv):
    print("current inventory: ")
    for item, quantity in inv.items():
        print(f"{item}: {quantity}")
    time.sleep(1)

def main():
    inv = {}

    while True:
        print("\n1. Add item to inventory")
        print("2. Remove item from inventory")
        print("3. Display inventory")
        print("4. Exit")
        choice = input("Enter your choice (1-4): ")

        if choice == '1':
            item_name = input("Enter item name: ")
            quantity = int(input("Enter quantity: "))
            add_item(inv, item_name, quantity)
        elif choice == '2':
            item_name = input("Enter item name: ")
            quantity = int(input("Enter quantity: "))
            remove_item(inv, item_name, quantity)
        elif choice == '3':
            display_inv(inv)
        elif choice == '4':
            print("Exiting program...")
            break
        else:
            print("Invalid choice. Please enter a number from 1 to 4.")

if __name__ == "__main__":
    main()