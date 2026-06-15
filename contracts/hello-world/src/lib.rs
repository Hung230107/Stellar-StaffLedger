#![cfg(test)]
use super::{EmployeeManagementContract, EmployeeManagementContractClient, Employee};
use soroban_sdk::{Env, String};

#[test]
fn test_employee_management_lifecycle() {
    // 1. Khởi tạo môi trường ảo giả lập Blockchain Stellar (Soroban Env)
    let env = Env::default();
    
    // Đăng ký Contract quản lý nhân viên vào môi trường ảo
    let contract_id = env.register_contract(None, EmployeeManagementContract);
    
    // Tạo một Client để tương tác và gọi các hàm của Contract
    let client = EmployeeManagementContractClient::new(&env, &contract_id);

    // 2. Thiết lập dữ liệu mẫu dạng String chuẩn cấu trúc Soroban SDK
    let emp_name = String::from_str(&env, "Nguyen Phi Hung");
    let emp_role = String::from_str(&env, "Developer");
    let updated_role = String::from_str(&env, "Tech Lead");

    // 3. KIỂM THỬ TÍNH NĂNG 1: Thêm nhân viên mới (add_employee)
    // Thêm nhân viên Nguyễn Phi Hùng, ID = 1, lương = 2500
    client.add_employee(&1, &emp_name, &emp_role, &2500);

    // 4. KIỂM THỬ TÍNH NĂNG 2: Lấy dữ liệu và đối soát dữ liệu (get_employee)
    let employee_record = client.get_employee(&1);
    assert_eq!(employee_record.id, 1);
    assert_eq!(employee_record.name, emp_name);
    assert_eq!(employee_record.role, emp_role);
    assert_eq!(employee_record.salary, 2500);

    // 5. KIỂM THỬ TÍNH NĂNG 3: Cập nhật thông tin (update_employee)
    // Thăng chức lên Tech Lead và tăng lương lên 4000
    client.update_employee(&1, &updated_role, &4000);
    
    // Đọc lại dữ liệu sau khi sửa xem Blockchain đã cập nhật đúng chưa
    let updated_record = client.get_employee(&1);
    assert_eq!(updated_record.role, updated_role);
    assert_eq!(updated_record.salary, 4000);

    // 6. KIỂM THỬ TÍNH NĂNG 4: Xóa dữ liệu nhân viên (delete_employee)
    client.delete_employee(&1);

    // Thử thách hệ thống: Đọc lại ID số 1 sau khi xóa. 
    // Vì hàm get_employee có lệnh panic nếu không tìm thấy, nên đoạn code dưới đây 
    // sử dụng cơ chế `should_panic` ngầm để chứng minh dữ liệu đã thực sự biến mất sạch sẽ.
    let result = env.try_invoke_contract::<Employee, soroban_sdk::Error>(
        &contract_id,
        &soroban_sdk::symbol_short!("get_employee"),
        soroban_sdk::vec![&env, 1.into_val(&env)],
    );
    assert!(result.is_err(), "Loi: Du lieu nhan vien dang le phai bi xoa hoan toan!");
}