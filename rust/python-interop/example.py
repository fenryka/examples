# Import the MyStruct class from your extension module
from python_interop import MyStruct

def main():
    # Create a new instance
    obj = MyStruct("Hello from Python!")
    print(f"Created object: {obj}")
    
    # Access the property
    print(f"Property value: {obj.a}")
    
    # Modify the property
    obj.a = "Modified in Python"
    print(f"After modification: {obj}")
    
    # Use the Rust-implemented method
    uppercase = obj.to_uppercase()
    print(f"Uppercase version: {uppercase}")
    
    # Create another instance
    obj2 = MyStruct("Another instance")
    print(f"Second object: {obj2}")
    
    # Compare the two objects
    print(f"Are they the same? {obj.a == obj2.a}")

if __name__ == "__main__":
    main()
